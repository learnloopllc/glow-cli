class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.4.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.4.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "571cd194d8f40e61bcdb997eb1ebdb24b6ed9af61bdb3f83a0ea54916846d07b"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.4.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "9527b60a25a655dbd233c769e18e9c42a7836124ff1c5b8acc53d07019bbceeb"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.4.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "b4379950e01849d8305130802c4d714c076e11ec7255922addac92cfb054548b"
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
