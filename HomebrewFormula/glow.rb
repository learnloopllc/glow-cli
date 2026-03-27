class Glow < Formula
  desc "CLI for the Glow platform — manage instances, deployments, and resources"
  homepage "https://github.com/learnloopllc/glow-cli"
  version "2.4.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.4.0/glow-aarch64-apple-darwin.tar.gz"
      sha256 "5a4ad06dfff5cbcc5f7ec84af27d0fac9bfdd1f2be2fec6fd7f7fa88fc9dda59"
    else
      url "https://github.com/learnloopllc/glow-cli/releases/download/v2.4.0/glow-x86_64-apple-darwin.tar.gz"
      sha256 "b3ac1f7d0589f6b80a66afe04d3d92ebc687d7fbbfac6675bcbe7d43dd18bda8"
    end
  end

  on_linux do
    url "https://github.com/learnloopllc/glow-cli/releases/download/v2.4.0/glow-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "790c32fc5ecf03c0c636a53697b1c191567c49cce24463e91f65e3c0d9d46f29"
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
