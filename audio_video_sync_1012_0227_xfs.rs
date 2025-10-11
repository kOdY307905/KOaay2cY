use tokio::sync::mpsc;
use tokio::time::{self, Duration};
# 扩展功能模块
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Instant;
# 优化算法效率

/// AudioVideoSync struct that holds the state for audio and video synchronization
pub struct AudioVideoSync {
    audio_packets: Arc<Mutex<VecDeque<(Instant, Vec<u8>)>>>,
    video_packets: Arc<Mutex<VecDeque<(Instant, Vec<u8>)>>>,
# 增强安全性
    sync_channel: mpsc::Sender<(Instant, Vec<u8>)>,
# 扩展功能模块
}

impl AudioVideoSync {
    /// Creates a new AudioVideoSync instance
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel(100);
        let audio_packets = Arc::new(Mutex::new(VecDeque::new()));
        let video_packets = Arc::new(Mutex::new(VecDeque::new()));

        tokio::spawn(async move {
            while let Some((timestamp, packet)) = rx.recv().await {
                Self::process_packet(timestamp, &packet, &audio_packets, &video_packets).await;
            }
        });

        AudioVideoSync {
            audio_packets,
            video_packets,
            sync_channel: tx,
        }
    }

    /// Processes a packet and synchronizes it with the other stream
    async fn process_packet(timestamp: Instant, packet: &[u8], audio_packets: &Arc<Mutex<VecDeque<(Instant, Vec<u8>)>>>, video_packets: &Arc<Mutex<VecDeque<(Instant, Vec<u8>)>>>) {
        let mut audio_stream = audio_packets.lock().await;
        let mut video_stream = video_packets.lock().await;

        if packet[0] == 1 { // Assuming the first byte denotes whether it's audio or video packet
            audio_stream.push_back((timestamp, packet.to_vec()));
# FIXME: 处理边界情况
        } else {
            video_stream.push_back((timestamp, packet.to_vec()));
        }

        while let Some((audio_timestamp, audio_packet)) = audio_stream.front() {
            let delay = (*audio_timestamp).saturating_duration_since(Instant::now());
            if delay > Duration::from_millis(100) {
                audio_stream.pop_front();
            } else {
                break;
            }
        }

        while let Some((video_timestamp, video_packet)) = video_stream.front() {
            let delay = (*video_timestamp).saturating_duration_since(Instant::now());
            if delay > Duration::from_millis(100) {
                video_stream.pop_front();
            } else {
                break;
            }
        }

        // Synchronize and output the audio and video packets
        if let (Some((audio_timestamp, audio_packet)), Some((video_timestamp, video_packet))) = (audio_stream.front(), video_stream.front()) {
            if audio_timestamp >= video_timestamp {
                Self::output_packet(&audio_packets, &video_packets, audio_timestamp, audio_packet).await;
            } else {
                Self::output_packet(&audio_packets, &video_packets, video_timestamp, video_packet).await;
            }
        }
    }

    /// Outputs a synchronized packet
    async fn output_packet(audio_packets: &Arc<Mutex<VecDeque<(Instant, Vec<u8>)>>>, video_packets: &Arc<Mutex<VecDeque<(Instant, Vec<u8>)>>>, timestamp: Instant, packet: Vec<u8>) {
        // Logic to output the synchronized packet, e.g., send it to a display or audio output
        println!("Synchronized packet at {}: {:?}