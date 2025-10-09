use tokio::time::{sleep, Duration};

// 定义一个结构体来封装异常检测的配置
struct AnomalyDetectorConfig {
    threshold: f32,
    // 异常检测的阈值
    window_size: usize,
    // 滑动窗口大小
}
anomaly_detection_config!();

// 实现异常检测算法的主要结构体
struct AnomalyDetector {
    config: AnomalyDetectorConfig,
    // 异常检测的配置
    current_window: Vec<f32>,
    // 保存当前窗口内的数据
    rolling_mean: f32,
    // 滚动平均值
    rolling_std_dev: f32,
    // 滚动标准差
}
anomaly_detector!();

impl AnomalyDetector {
    // 创建一个新的异常检测器实例
    fn new(config: AnomalyDetectorConfig) -> Self {
        AnomalyDetector {
            config,
            current_window: Vec::with_capacity(config.window_size),
            rolling_mean: 0.0,
            rolling_std_dev: 0.0,
        }
    }

    // 添加新的数据点到异常检测器
    async fn add_data_point(&mut self, value: f32) -> Result<(), String> {
        // 检查数据是否超出阈值
        if value > self.rolling_mean + self.config.threshold * self.rolling_std_dev {
            return Err("Value exceeds the anomaly threshold".to_string());
        }

        // 更新滚动平均值和标准差
        self.update_rolling_stats(value).await;

        // 将新值添加到窗口并检查窗口大小
        if self.current_window.len() == self.config.window_size { {