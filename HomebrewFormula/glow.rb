class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.2.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "83a92d8cdea81d66d0212515f7c4fc3790f1e5509aff858591b0d003003d418e"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "97e59736ca6577aaea1453596d37263e9f7ac67e92882067c279473171cf07d6"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "3e2fc81590e4a32fcac2f57f28d11d1a1242ee0d2a6067c831da6e2303766343"
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
