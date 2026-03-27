class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.1.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "e545bca829cde79993834a2290cb4bcc105ed0f299f705bf40c5c8c03573ba8b"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.1.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "5b961bbc9952ce45bea1507adf3668f150cb5d486cc90edaa57a9ae97476f55a"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.1.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "7bd0b5c6e0bed42d46dab6128787f139c97b7b949a49a1e4f87eb6f64e247e9b"
  end

  def install
    bin.install "glow"
    bin.install "glw"

    # Generate shell completions
    generate_completions_from_executable(bin/"glow", "completions")
  end

  test do
    assert_match "Glow CLI", shell_output("#{bin}/glow --help")
    assert_match version.to_s, shell_output("#{bin}/glow --version")
    assert_match "Glow CLI", shell_output("#{bin}/glw --help")
  end
end
