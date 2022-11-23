mod string_analysis;

use string_analysis::Analysis;

fn main() {
    let analysis = Analysis::new("Today is Sunday. For the first time they took me out into the sun today. And for the first time in my life I was aghast, that the sky is so far away, and so blue, and so vast!");
    analysis.output_report();
}
