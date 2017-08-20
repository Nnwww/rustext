use clap::{App, ArgGroup, Arg as CLArg};
use std::path::PathBuf;

arg_enum! {
/// Learning methods
pub enum Method { Cbow, Skipgram }
}

arg_enum! {
/// Loss functions
pub enum Loss { Negative, Hierarchical }
}
/// Arguments necessary to learn
pub struct Arg {
    /// training file path
    input : PathBuf,
    /// output file path
    output : PathBuf,
    /// learning method {cbow, skipgram}
    method : Method,
    /// loss function {negative sampling, hierarchical softmax}
    loss : Loss,
    /// the initial learning rate
    init_lr : f64,
    /// the number of tokens that update the learning rate
    lr_update_rate : u64,
    /// size of word vectors
    dim : u32,
    /// size of the context window
    windows : u32,
    /// number of epochs
    epoch : u32,
    /// minimal number of word occurrences
    min_count : u32,
    /// number of negatives sampled
    negatives : u32,
    /// sub-sampling threshold
    t_sub : f64,
    /// number of threads
    threads : u32,
    /// verbosity level
    verbose : u8,
}

fn parse() -> Arg {
    let matches = App::new("rustext")
        .version("0.1.0")
        .author("Nnwww <johndororo@gmail.com>")
        .about("Rust word2vec implementation based on fasttext")
        .arg(CLArg::with_name("INPUT")
            .help("input courpus path")
            .required(true)
            .index(1))
        .arg(CLArg::with_name("output")
            .short("o")
            .long("output")
            .value_name("PATH")
            .help("output path of the learning model. (default: INPUT ++ .out)")
            .takes_value(true))
        .group(ArgGroup::with_name("method")
            .args(&["cbow", "skipgram"])
            .required(true))
        .group(ArgGroup::with_name("loss")
            .args(&["negative", "hierarchical"])
            .required(true))
        .arg(CLArg::with_name("init_lr")
            .short("lr")
            .long("init_lr")
            .value_name("FLOAT")
            .help("initial learning rate")
            .takes_value(true))
        .arg(CLArg::with_name("lr_update_rate")
            .short("lur")
            .long("lr_update_rate")
            .value_name("UINT")
            .help("the number of tokens that update the learning rate")
            .takes_value(true))
        .arg(CLArg::with_name("dim")
            .short("d")
            .long("dim")
            .value_name("UINT")
            .help("size of word vectors")
            .takes_value(true))
        .arg(CLArg::with_name("windows")
            .short("w")
            .long("windows")
            .value_name("UINT")
            .help("size of the context windows")
            .takes_value(true))
        .arg(CLArg::with_name("epoch")
            .short("e")
            .long("epoch")
            .value_name("UINT")
            .help("size of the context windows")
            .takes_value(true))
        .arg(CLArg::with_name("min_count")
            .short("c")
            .long("min_count")
            .value_name("UINT")
            .help("minimal number of word occurrences")
            .takes_value(true))
        .arg(CLArg::with_name("negatives")
            .short("neg")
            .long("negatives")
            .value_name("UINT")
            .help("number of negatives sampled")
            .takes_value(true))
        .arg(CLArg::with_name("t_sub")
            .short("ts")
            .long("t_sub")
            .value_name("FLOAT")
            .help("sub-sampling threshold")
            .takes_value(true))
        .arg(CLArg::with_name("threads")
            .short("th")
            .long("threads")
            .value_name("UINT")
            .help("number of threads")
            .takes_value(true))
        .arg(CLArg::with_name("verbose")
            .short("v")
            .long("verbose")
            .value_name("UINT")
            .takes_value(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let input_buf;
    {
        let input_path = matches.value_of("INPUT").unwrap();
        input_buf = PathBuf::from(input_path);
    }

    let output_buf = match matches.value_of("output") {
        Some(output_path) => PathBuf::from(output_path),
        None => input_buf.with_extension("out"),
    };

    Arg {
        input : input_buf,
        output : output_buf,
        method : value_t!(matches, "method", Method).unwrap_or_else(|e| e.exit()),
        loss : value_t!(matches, "loss", Loss).unwrap_or_else(|e| e.exit()),
        init_lr : value_t!(matches, "init_lr", f64).unwrap_or_else(|e| e.exit()),
        lr_update_rate : value_t!(matches, "lr_update_rate", u64).unwrap_or_else(|e| e.exit()),
        dim : value_t!(matches, "dim", u32).unwrap_or_else(|e| e.exit()),
        windows : value_t!(matches, "windows", u32).unwrap_or_else(|e| e.exit()),
        epoch : value_t!(matches, "epoch", u32).unwrap_or_else(|e| e.exit()),
        min_count : value_t!(matches, "min_count", u32).unwrap_or_else(|e| e.exit()),
        negatives : value_t!(matches, "min_count", u32).unwrap_or_else(|e| e.exit()),
        t_sub : value_t!(matches, "t_sub", f64).unwrap_or_else(|e| e.exit()),
        threads : value_t!(matches, "threads", u32).unwrap_or_else(|e| e.exit()),
        verbose : value_t!(matches, "threads", u8).unwrap_or_else(|e| e.exit()),
    }
}