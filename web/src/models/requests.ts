export enum ExploreDataCategory {
  Demographics = 'Demographics',
  MentalHealth = 'MentalHealth',
}

export const ExploreDataCategoryMapping: Record<ExploreDataCategory, string> = {
    [ExploreDataCategory.Demographics]: "Demographics",
    [ExploreDataCategory.MentalHealth]: "Mental Health",
};

export enum DemographicCategory {
  ADHDSubtypesByAgeGroup = 'ADHDSubtypesByAgeGroup',
  ADHDSubtypesByGender = 'ADHDSubtypesByGender'
}

export const DemographicCategoryMapping: Record<DemographicCategory, string> = {
  [DemographicCategory.ADHDSubtypesByAgeGroup]: "Data By Age Group",
  [DemographicCategory.ADHDSubtypesByGender]: "Data By Gender"
};

export enum MentalHealthCategory {
        HasCoMorbidMentalHealthCondition = "HasCoMorbidMentalHealthCondition",
        HasBipolarDisorder = "HasBipolarDisorder",
        HasUnipolarDepression = "HasUnipolarDepression",
        HasAnxiety = "HasAnxiety",
        HasSubstanceAbuseDisorder = "HasSubstanceAbuseDisorder",
        HasOther = "HasOther"
}

export const MentalHealthCategoryMapping: Record<MentalHealthCategory, string> = {
  [MentalHealthCategory.HasCoMorbidMentalHealthCondition]: "All",
  [MentalHealthCategory.HasBipolarDisorder]: "Bipolar Disorder",
  [MentalHealthCategory.HasUnipolarDepression]: "Unipolar Depression",
  [MentalHealthCategory.HasAnxiety]: "Anxiety",
  [MentalHealthCategory.HasSubstanceAbuseDisorder]: "Substance Abuse Disorder",
  [MentalHealthCategory.HasOther]: "Other"
};


export enum VisualizationOptions {
  Graph = 'Graph', // TODO: BarChart, Scatterplot, etc.
  Table = 'Table'
}

export const VisualizationOptionsMapping: Record<VisualizationOptions, string> = {
  [VisualizationOptions.Graph]: "Graph",
  [VisualizationOptions.Table]: "Table"
}

export class DemographicsRequest {
  constructor(
    public display: string,
    public sub_category: string,
    public gender: string | null = null,
    public with_controls: boolean = false
  ){}
}

export class MentalHealthRequest {
  constructor(
    public display: string,
    public sub_category: string,
    public gender: string | null = null,
    public with_controls: boolean = false
  ){}
}
