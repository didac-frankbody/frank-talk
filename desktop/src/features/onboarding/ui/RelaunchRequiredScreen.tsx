import { RecoveryScreen } from "./RecoveryScreen";

export function RelaunchRequiredScreen() {
  return (
    <RecoveryScreen
      testId="relaunch-required"
      title="Restart frank talk to finish recovery"
      body="Your identity was updated. frank talk needs to restart so syncing and agents run under it."
    />
  );
}
