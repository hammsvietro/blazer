defmodule Blazer.MixProject do
  use Mix.Project

  @source_url "https://github.com/hammsvietro/blazer"
  @version "0.1.0"

  def project do
    [
      app: :blazer,
      version: @version,
      elixir: "~> 1.8",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      docs: docs(),
      package: package()
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.23.0"},
      {:jason, "~> 1.3"},
      {:credo, "~> 1.5", only: [:dev, :test], runtime: false}
    ]
  end

  defp package do
    [
      description: "A blazingly fast case parser for maps and strings",
      files: ["lib", "mix.exs", "README*", "CHANGELOG*", "LICENSE*"],
      maintainers: ["Pedro Hamms Vietro"],
      licenses: ["MIT"],
      links: %{
        "Changelog" => "https://hexdocs.pm/blazer/changelog.html",
        "GitHub" => @source_url
      }
    ]
  end

  defp docs do
    [
      extras: [
        "README.md": [title: "Overview"],
        "CHANGELOG.md": [title: "Changelog"],
        "LICENSE.md": [title: "License"]
      ],
      main: "readme",
      source_url: @source_url,
      source_ref: "v#{@version}",
      formatters: ["html"]
    ]
  end
end
