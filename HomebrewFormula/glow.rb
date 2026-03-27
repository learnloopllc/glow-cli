class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.5.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.5.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "9a3a7c0ac032d8c9b2657b102ec15f1a06385b7e2d35990942c80a725dd49200"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.5.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "5712c14ac326962537cf3e6ecf3ed96b0dedc96fabb7eeb46e2f280e33ec6bbb"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.5.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "63389526d31f741808c4745bc50751ba538db51b5b2cfae157f96950661463de"
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
