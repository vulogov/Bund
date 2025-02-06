use anomaly_detection;
use easy_error::{Error, bail};

pub fn detect_anomalies(source: Vec<f64>, period: usize) -> Result<Vec<f64>, Error> {
    let mut src: Vec<f32> = Vec::new();
    let mut dst: Vec<f64> = Vec::new();
    for i in &source {
        src.push(*i as f32);
    }
    let _ = match anomaly_detection::AnomalyDetector::fit(&src, period) {
        Ok(res_ix) => {
            for ix in res_ix.anomalies() {
                dst.push(src[*ix].into());
            }
        }
        Err(err) => bail!("ANOMALY DETECTION returns error: {}", err),
    };
    Ok(dst)
}
