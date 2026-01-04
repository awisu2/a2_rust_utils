use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::{Mutex, Notify, Semaphore};

// 実行するジョブ（共有・スレッド安全）
// dyn: 動的ディスパッチ: 実行時に型を決定する仕組み
// Fn(): クロージャトレイト: 引数なし、戻り値なしの関数型
// Send: スレッド間で所有権を移動できる型の制約
// Sync: 複数のスレッドから同時に参照できる型の制約
// 'static: プログラム全体のライフタイムを持つ型の制約
type Job = Arc<dyn Fn() + Send + Sync + 'static>;

pub struct JobExecutor {
    // ジョブキュー（非同期・排他制御）
    queue: Arc<Mutex<VecDeque<Job>>>,

    // 同時実行数制限（ここでは最大10）
    sem: Arc<Semaphore>,

    // ワーカーが起動中かどうかのフラグ（多重起動防止）
    running: AtomicBool,
    // キューにジョブが追加されたことを通知するためのシグナル
    notify: Arc<Notify>,
}

impl JobExecutor {
    // 初期化
    pub fn new(thread_limit: usize) -> Arc<Self> {
        Arc::new(Self {
            sem: Arc::new(Semaphore::new(thread_limit)),

            queue: Arc::new(Mutex::new(VecDeque::new())),
            running: AtomicBool::new(false),
            notify: Arc::new(Notify::new()),
        })
    }

    // ワーカー起動（何度呼んでもOK）
    pub fn start(self: Arc<Self>) {
        // すでに起動中なら何もしない
        if self
            .running
            // 現在値が第1引数と一致した場合に、第2引数へ原子的に更新する。
            // 第3・第4引数は「この操作を境に、メモリが他スレッドからどう見えるか」を指定する。
            //
            // 各スレッドは動作時に専用のcpuキャッシュを持ち見えている値が異なる(通常は最新に同期される)。
            // cpuの特性として全体の処理を早めるために、軽い処理(読み込みなど)は先に行い、遅い処理(書き込みなど)は一旦ストアバッファに溜め込みあとでまとめて反映する。
            // この時間差により、記述した順序とは関係なく値の更新が他スレッドから即座に見えないことがある。
            //
            // AcqRel（成功時）:
            //   値を更新し、この操作より前に行った書き込みも含めて
            //   他スレッドから必ず見える状態にする。
            //
            // Acquire（失敗時）:
            //   値は更新せず、他スレッドが公開した最新状態を
            //   正しく観測できるようにする。
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }

        tauri::async_runtime::spawn(async move {
            loop {
                // ジョブ取得ループ
                let job = loop {
                    // キューから1件取得
                    if let Some(j) = self.queue.lock().await.pop_front() {
                        break j;
                    }
                    // 空なら通知が来るまで待機
                    self.notify.notified().await;
                };

                // lock and acquire semaphore permit
                // acquire: 確保
                // permit: 許可
                let permit = match self.sem.clone().acquire_owned().await {
                    Ok(p) => p,
                    Err(_) => return,
                };

                // 実ジョブ実行（CPUブロッキング想定）
                tauri::async_runtime::spawn(async move {
                    let _permit = permit; // スコープ終了まで保持
                    let _ = tauri::async_runtime::spawn_blocking(move || {
                        (job)();
                    })
                    .await;
                });

                // キューが空になったらワーカー停止
                if self.queue.lock().await.is_empty() {
                    self.running.store(false, Ordering::Release);
                    break;
                }
            }
        });
    }

    pub async fn push(&self, job: Box<dyn Fn() + Send + Sync + 'static>) {
        {
            let mut q = self.queue.lock().await;
            q.push_back(Arc::from(job));
        }

        // 待機中ワーカーを起こす
        self.notify.notify_one();
    }

    // ジョブ追加（先頭にまとめて追加）
    pub async fn push_front_list(&self, jobs: Vec<Box<dyn Fn() + Send + Sync + 'static>>) {
        {
            let mut q = self.queue.lock().await;
            // 先頭順を保つため reverse して push_front
            for job in jobs.into_iter().rev() {
                q.push_front(Arc::from(job));
            }
        }

        // 待機中ワーカーを起こす
        self.notify.notify_one();
    }
}

// テスト用実装
#[cfg(test)]
mod tests {
    use super::JobExecutor;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_job_executor() {
        let runner = JobExecutor::new(3);
        let counter = Arc::new(AtomicUsize::new(0));
        runner.clone().start();

        // ジョブを10件追加
        for _ in 0..10 {
            let c = counter.clone();
            runner
                .push(Box::new(move || {
                    let prev = c.fetch_add(1, Ordering::SeqCst);
                    println!("Job started: {}", prev + 1);
                    // 擬似的な重い処理
                    std::thread::sleep(Duration::from_millis(100));
                    println!("Job finished: {}", prev + 1);
                }))
                .await;
        }

        // 全ジョブ完了まで待機
        loop {
            if counter.load(Ordering::SeqCst) >= 10 {
                break;
            }
            sleep(Duration::from_millis(50)).await;
        }
    }
}
