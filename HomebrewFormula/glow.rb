class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.8.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "c914b45ef7f81360e52dff810de60ee0213862002b1033631826d32cd39e04e0"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "6afde6281897060d018186300188242ab5355a521b08246d3af8baa967f04e93"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "b5efa749dbb97bfe5ca78b6b976f31fdba157631f8b6b80d0e8398dab5190348"
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
