class GlowBeta < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.8.0"
  license "MIT"

  conflicts_with "glow", because: "both install glow and glw binaries"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0-beta/glow-aarch64-apple-darwin.tar.gz"
      sha256 "ed963fbcacbc60fba6df0b79b1448536cfad7c5ff5d5df13bd24518809899cd7"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0-beta/glow-x86_64-apple-darwin.tar.gz"
      sha256 "7b4ab57221ff388127c539e8a186f7f092a8ba90f5ce8ea94effac4f58c1a32e"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0-beta/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "f03b78e33619fce44fcea24bb9a286946283026e7536bcb48da857913e4ec7aa"
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
