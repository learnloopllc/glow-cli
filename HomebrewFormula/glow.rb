class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.3.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.3.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "50f93cef9a2403dbeff6dc70c56bb92955d0e79b5b30ce31deb9be2b485e9e60"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.3.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "fbc13a62cb5e9b401b1ed53878d1a03d54d32a904d6afabdb8bcc5d646e1e054"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.3.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "1497ca19e84433ab2d206f27aa740c383c47650d82bbb1d7f75fda2ea776bd45"
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
