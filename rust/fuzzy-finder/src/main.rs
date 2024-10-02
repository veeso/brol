use std::path::PathBuf;

use argh::FromArgs;
use nucleo::Utf32String;

#[derive(FromArgs, Debug)]
/// Fuzzy search
struct CliArgs {
    /// minikube ip; if not provided it will be fetched using `minikube ip`
    #[argh(option, default = "PathBuf::from(\".\")")]
    path: PathBuf,
    /// score threshold
    #[argh(option, default = "0")]
    threshold: u16,
    /// search pattern
    #[argh(positional)]
    search: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let CliArgs {
        path,
        search,
        threshold,
    } = argh::from_env::<CliArgs>();
    let paths: Vec<Utf32String> = walkdir::WalkDir::new(&path)
        .into_iter()
        .filter_map(|path| {
            let dent = path.ok()?;
            let path = dent.into_path().to_string_lossy().into_owned();
            Some(path.as_str().into())
        })
        .collect();

    let search: Utf32String = search.into();

    let mut nucleo = nucleo::Matcher::new(nucleo::Config::DEFAULT.match_paths());

    // get scores; the highest the score, the best the match
    let mut path_scores = paths
        .into_iter()
        .filter_map(|path| {
            nucleo
                .fuzzy_match(path.slice(..), search.slice(..))
                .map(|score| (path, score))
        })
        .filter(|(_, score)| score >= &threshold)
        .collect::<Vec<_>>();
    path_scores.sort_by(|a, b| b.1.cmp(&a.1));

    let paths: Vec<_> = path_scores.into_iter().map(|(path, _)| path).collect();

    for path in paths.iter() {
        println!("{path}");
    }

    Ok(())
}
