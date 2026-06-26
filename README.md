# monitoring-runner

CLI launcher for LocalMonitoring Standalone. Sets required environment variables and starts the process.

## Build

```bash
cargo build --release
```

## Usage

```bash
monitoring-runner --lm-bin-path <PATH> [OPTIONS]
```

### Options

| Flag | Default | Description |
|------|---------|-------------|
| `--lm-bin-path` | *(required)* | Path to `LocalMonitoringStandalone` executable |
| `--grpc-port` | `20109` | gRPC port for vms (`LOCALMONITORING_VMS_ENDPOINT`) |
| `--prometheus-port` | `20400` | Prometheus port (`LOCALMONITORING_PROMETHEUS_URL`) |
| `--lm-port` | `9999` | Port LocalMonitoring listens on (`LOCALMONITORING_PORT`) |
| `--lm-host` | `localhost` | Hostname LocalMonitoring binds to (`HOSTNAME`) |
| `--ticket-dir` | `C:\ProgramData\AxxonSoft\AxxonNext\Tickets` (Windows) | Path to Tickets folder (`NGP_SECRETS_DIR`) |
| `--db-dir` | `.` | Plugin DB path (`NGP_PLUGINS_DATA_PATH`) |
| `--token` | *(auto via ngpsh)* | NBL token (`LOCALMONITORING_NBL_TOKEN`) |

### Example

```bash
monitoring-runner \
  --lm-bin-path ./LocalMonitoringStandalone \
  --grpc-port 20109 \
  --prometheus-port 20040 \
  --token eyJhbGc...
```
