# C964 - Capstone for WGU Computer Science Program

## Project Overview

This project will be the last submitted at WGU to earn my Bachelor of Science in Computer Science. I have chosen to use this opportunity to explore the data science & machine learning ecosystem that is currently present in the Rust community.

Note that I'm still working on this - as long as this comment is present, there will be continuing updates to this repository.

For the evaluator:

### Project Folder Structure

- `.github` - Contains GitHub actions workflows to build/test/lint the project. 
- `data` - Contains data files - specifically, the `hyperaktiv` dataset, sourced from [Kaggle](https://www.kaggle.com/datasets/arashnic/adhd-diagnosis-data)
- `docs` - Contains documentation artifacts
- `src` - Contains Rust source code for the project.
    - `algo` - Contains application of machine learning algorithms.
    - `api` - Contains API handlers for axum.
    - `enums` - Contains enum mappings & trait impl for Hyperaktiv dataset
    - `frames` - Contains data frames built on top of the `hyperaktiv` data. This is where data is enriched through cleaning & transformations.'
    - `http` - Contains elements for API interop - request & response models + trait implementations
    - `plots` - Contains a collection of visualizations built on top of `frames` and `experiments`
    - `predict` - Contains machine learning model training implementations.
    - `traits` - Contains Rust traits used to extend dataframes - clean, filter, translate, and query data.
- `web` - Contains Angular web application serving as frontend for the project.


## Rust Crates Used For Data Product

- [Linfa](https://github.com/rust-ml/linfa) is used to apply machine learning algorithms. This is standing in for Python's `scikit-learn`, in this context.
- [Polars](https://github.com/pola-rs/polars) is used for IO & DataFrame processing. This fills the functionality `pandas` would provide in a Python environment.
- [Plotlars](https://github.com/alceal/plotlars) is used for visualizations of data frames. Plotlars, specifically, is used to bridge functionality between the `plotly` and `polars` crates, greatly simplifying the process of converting `polars` data frames into graphical formats. This crate provides what the `matplotlib` library would in a Python environment.

## Frameworks and Libraries used for Front End

- [Angular](https://angular.dev/) with [Typescript](https://www.typescriptlang.org/) was used to build the frontend project, contained in the `web` folder.
- [PlotlyJS](https://plotly.com/javascript/) is used for the visualizations. This works in conjunction with the backend Plotlars crate to show the data visually.
- [BulmaCSS](https://bulma.io/) was used to style the application.
