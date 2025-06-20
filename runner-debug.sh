#!/usr/bin/env bash
# Debug script for GitHub Actions self-hosted runner

echo "=== GitHub Actions Runner Debug ==="
echo

# Check if runner service is running
echo "1. Checking runner service status:"
sudo systemctl status actions.runner.* 2>/dev/null || echo "No systemd service found"
echo

# Check for runner processes
echo "2. Checking for runner processes:"
ps aux | grep -i runner | grep -v grep || echo "No runner processes found"
echo

# Check runner directory and logs
echo "3. Checking common runner locations:"
for dir in /opt/actions-runner ~/actions-runner /home/*/actions-runner; do
    if [[ -d "$dir" ]]; then
        echo "Found runner at: $dir"
        if [[ -f "$dir/_diag/Runner_*.log" ]]; then
            echo "Recent log entries:"
            tail -20 "$dir"/_diag/Runner_*.log | tail -10
        fi
        echo
    fi
done

# Check network connectivity to GitHub
echo "4. Testing GitHub connectivity:"
curl -s -o /dev/null -w "GitHub API: %{http_code}\n" https://api.github.com || echo "GitHub API unreachable"
echo

echo "5. Runner registration status:"
echo "If runner is registered but not running, try:"
echo "  cd /path/to/actions-runner"
echo "  ./run.sh"
echo
echo "If runner needs to be registered:"
echo "  ./config.sh --url https://github.com/ghostkellz/oxygen --token YOUR_TOKEN"
echo