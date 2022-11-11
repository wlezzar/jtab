# Generated with JReleaser 1.3.1 at 2022-11-11T22:38:59.461246635Z
class Jtab < Formula
  desc "Print any json data as a table from the command line"
  homepage "https://github.com/wlezzar/jtab"
  version "0.6.0"
  license "MIT"

  if OS.linux? && Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
    url "https://github.com/wlezzar/jtab/releases/download/v0.6.0/jtab-0.6.0-aarch64-unknown-linux-gnu.zip"
    sha256 "3321e1d3aa3262a243042fe42535cd1c3b250aadfcaa6f680a530e7246a1beab"
  end
  if OS.linux? && Hardware::CPU.intel?
    url "https://github.com/wlezzar/jtab/releases/download/v0.6.0/jtab-0.6.0-x86_64-unknown-linux-gnu.zip"
    sha256 "802e2ed608dbd18db30925cb7ef6b8a511e248a1d0286c1a35d5766ce35d82aa"
  end
  if OS.mac? && Hardware::CPU.arm?
    url "https://github.com/wlezzar/jtab/releases/download/v0.6.0/jtab-0.6.0-aarch64-apple-darwin.zip"
    sha256 "c95e29f869af4397f7424acfb7328ee69030b0b367ec35e7a4debfd1b2fdf291"
  end
  if OS.mac? && Hardware::CPU.intel?
    url "https://github.com/wlezzar/jtab/releases/download/v0.6.0/jtab-0.6.0-x86_64-apple-darwin.zip"
    sha256 "40ad59ec0ef64ec57381c1755096e406089cbfee90d69d3101628a72498959b1"
  end


  def install
    libexec.install Dir["*"]
    bin.install_symlink "#{libexec}/bin/jtab" => "jtab"
  end

  test do
    output = shell_output("#{bin}/jtab --version")
    assert_match "0.6.0", output
  end
end
