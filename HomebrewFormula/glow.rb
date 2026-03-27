class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.7.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.7.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "027fd8716c5c3faf73aa7739aac5926db9e7f8d71840f451ba5469aa55a9da28"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.7.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "7b71500b6191a0d05619490a0f89020898b8e4d554c5047e851e5e9fcdbc399c"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.7.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "85092bc469e1643678b08af383a13cb939e50a3ca2d4d08eec96312452113865"
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
