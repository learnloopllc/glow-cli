class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.2.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "b3014b8011b89cd11126fbca9cfd5b39bd3a5302d1aae7a2eed2d5aea283732b"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "99b9cb0264bb9b70384016129bb1901913a20903cb72dc42d042ec06b1cd1fa4"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.2.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "4e9d601dd8146a5316eb76ca981af65a1d34ec4353e007af066e2e17aea6f2dd"
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
