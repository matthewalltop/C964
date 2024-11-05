export enum ExploreDataCategory {
  ADHDSubtypes = 'ADHDSubtypes',
  Demographics = 'Demographics',
  MentalHealth = 'MentalHealth',
  Medication = 'Medication'
}

export const ExploreDataCategoryMapping: Record<ExploreDataCategory, string> = {
    [ExploreDataCategory.ADHDSubtypes]: "ADHD Subtypes",
    [ExploreDataCategory.Demographics]: "Demographics",
    [ExploreDataCategory.MentalHealth]: "Mental Health",
    [ExploreDataCategory.Medication]: "Medication"
};

export enum VisualizationOptions {
  Graph = 'Graph', // TODO: BarChart, Scatterplot, etc.
  Table = 'Table'
}

export const VisualizationOptionsMapping: Record<VisualizationOptions, string> = {
  [VisualizationOptions.Graph]: "Graph",
  [VisualizationOptions.Table]: "Table"
}


class ApiRequest<T> {
  constructor(public category: ExploreDataCategory,){}
}
