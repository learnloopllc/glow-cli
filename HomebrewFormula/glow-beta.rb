class GlowBeta < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.9.0"
  license "MIT"

  conflicts_with "glow", because: "both install glow and glw binaries"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.9.0-beta/glow-aarch64-apple-darwin.tar.gz"
      sha256 "41f7f4680ec16310898fe1c9c7cb0cd5fecd18954c3dcb5b8a074e1c58102d05"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.9.0-beta/glow-x86_64-apple-darwin.tar.gz"
      sha256 "75ee61bdfb839fbea755d31984691d90b2c3ba361cd5f7568d8fd5de2f7d7000"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.9.0-beta/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "87fdfe32cd357b158fbcac034fa3332f85cc3b77e97d2283b246413620ff5b50"
  end

  def install
    bin.install "glow"
    bin.install "glw"

    generate_completions_from_executable(bin/"glow", "completions")
  end

  test do
    assert_match "Glow CLI", shell_output("#{bin}/glow --help")
    assert_match version.to_s, shell_output("#{bin}/glow --version")
    assert_match "Glow CLI", shell_output("#{bin}/glw --help")
  end
end
