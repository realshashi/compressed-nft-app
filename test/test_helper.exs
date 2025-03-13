ExUnit.start()

# Configure test environment
Application.ensure_all_started(:bubblegum)

# Set up environment for testing
System.put_env("RUST_LOG", "debug")
System.put_env("RUST_BACKTRACE", "1")
System.put_env("MIX_ENV", "test")

Logger.configure(level: :debug)