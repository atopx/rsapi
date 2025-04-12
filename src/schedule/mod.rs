use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobScheduler;
use tokio_cron_scheduler::JobSchedulerError;
use tracing::info;

pub async fn start(crontab: &crate::config::Crontab) -> Result<(), JobSchedulerError> {
    let mut sched = JobScheduler::new().await?;
    sched
        .add(Job::new_async_tz(
            &crontab.example,
            chrono::Local,
            |_id: uuid::Uuid, _scheduler: JobScheduler| {
                let trace_id = uuid::Uuid::new_v4().to_string();
                Box::pin(async move { example(&trace_id).await })
            },
        )?)
        .await?;

    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            info!("shutdown scheduler, bye!");
        })
    }));

    sched.start().await?;

    Ok(())
}

#[tracing::instrument]
pub async fn example(trace_id: &str) {
    info!(trace_id, "example task");
}
