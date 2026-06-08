# meta-signal-agent

Meta signal contract for privileged `agent` policy commands.

This crate owns owner-issued orders for agent spawning and retirement, lane
backend policy, backend configuration, and staged routing through the agent
front door. Ordinary router-facing traffic belongs in `signal-agent`.
