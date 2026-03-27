class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.8.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "b416df0c74fee6fedb2d972382a45b02a4623ca16c73f23d939ba647260afddc"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "9b8ae1081f9140d12c7f15a55dc7f871adf582c9a349e869c2e524a7bda70c06"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.8.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "4879385307147b7658fb325c2bbeb5a48b1af5ce393ebabdb0cb456620b8aad9"
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
