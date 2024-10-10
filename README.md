# C964 - Capstone for WGU Computer Science Program

## Project Overview

This project will be the last submitted at WGU to earn my Bachelor of Science in Computer Science. I have chosen to use this opportunity to explore the data science & machine learning ecosystem that is currently present in the Rust community.

I've done my best to organize the project using a uni-directional data flow, with the starting raw files being enriched by filtering, translations, and composition into usable frames before being curated by subsequent processes and passed off to machine learning algorithms.
There is a more detailed description of this, in practice, given the specific folder structure of the project below.

Note that I'm still working on this - as long as this comment is present, there will be continuing updates to this repository.

### Project Folder Structure

- `.github` - Contains GitHub actions workflows to build/test/lint the project. 
- `docs` - Contains documentation artifacts
- `hyperaktiv` - Contains the `hyperaktiv` dataset, sourced from [Kaggle](https://www.kaggle.com/datasets/arashnic/adhd-diagnosis-data)
- `src` - Contains Rust source code for the project.
    - `experiments` - Uses the underlying population samples derived within `frames` to perform experiments and train machine learning models. 
    - `frames` - Contains data frames built on top of the `hyperaktiv` data. This is where data is enriched through cleaning & transformations.'




## Rust Crates Used

- [Linfa](https://github.com/rust-ml/linfa) is used to apply machine learning algorithms. This is standing in for Python's `scikit-learn`, in this context.
- [Polars](https://github.com/pola-rs/polars) is used for IO & DataFrame processing. This fills the functionality `pandas` would provide in a Python environment.
- [Plotlars](https://github.com/alceal/plotlars) is used for visualizations of data frames. Plotlars, specifically, is used to bridge functionality between the `plotly` and `polars` crates, greatly simplifying the process of converting `polars` data frames into graphical formats. This crate provides what the `matplotlib` library would in a Python environment.  