defmodule MplBubblegum.MixProject do
  use Mix.Project

  @version "0.1.0"
  @description "Elixir NIFs for Metaplex Bubblegum compressed NFTs"

  def project do
    [
      app: :mpl_bubblegum,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      compilers: Mix.compilers(),
      deps: deps(),
      package: package(),
      description: @description,
      docs: docs()
    ]
  end

  defp package do
    [
      maintainers: ["Your Name"],
      licenses: ["Apache-2.0"],
      links: %{"GitHub" => "https://github.com/yourusername/mpl-bubblegum-elixir"},
      files: ~w(lib native mix.exs README.md LICENSE)
    ]
  end

  defp docs do
    [
      main: "MplBubblegum",
      source_ref: "v#{@version}",
      source_url: "https://github.com/yourusername/mpl-bubblegum-elixir"
    ]
  end

  def application do
    [extra_applications: [:logger, :crypto]]
  end

  defp deps do
    [
      {:rustler, "~> 0.30", runtime: false},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false},
      {:jason, "~> 1.4"},
      # Try an earlier version
      {:mox, "~> 1.0", only: :test}
    ]
  end
end
