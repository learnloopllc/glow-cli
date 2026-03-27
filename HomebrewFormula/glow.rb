class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.6.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.6.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "08c2670bf823316be63088c5a297254a020b6649b8637843c41ba3c993597706"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.6.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "1ff8f29c2652229b4a5cd7afd835beff11d18472f8b5a38e0315b8d1cdfdddbf"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.6.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "24486997eded76934f011db812c3f8197632758ca49d8d57d644fa4510484689"
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
