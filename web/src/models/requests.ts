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
        All = "All",
        HasBipolarDisorder = "BIPOLAR",
        HasUnipolarDepression = "UNIPOLAR",
        HasAnxiety = "ANXIETY",
        HasSubstanceAbuseDisorder = "SUBSTANCE",
        HasOther = "OTHER"
}

export const MentalHealthCategoryMapping: Record<MentalHealthCategory, string> = {
  [MentalHealthCategory.All]: "All",
  [MentalHealthCategory.HasBipolarDisorder]: "Bipolar Disorder",
  [MentalHealthCategory.HasUnipolarDepression]: "Unipolar Depression",
  [MentalHealthCategory.HasAnxiety]: "Anxiety",
  [MentalHealthCategory.HasSubstanceAbuseDisorder]: "Substance Abuse Disorder",
  [MentalHealthCategory.HasOther]: "Other"
};

export enum VisualizationOptions {
  Graph = 'Graph',
  Table = 'Table'
}

export const VisualizationOptionsMapping: Record<VisualizationOptions, string> = {
  [VisualizationOptions.Graph]: "Graph",
  [VisualizationOptions.Table]: "Table"
}

export enum Gender {
  All = 'All',
  Male = 'Male',
  Female = 'Female'
}

export const GenderMapping: Record<Gender, string> = {
  [Gender.All]: "All",
  [Gender.Female]: "Female",
  [Gender.Male]: "Male"
}

export enum AdhdType {
  All = 'All',
  ADHDPH = 'ADHD-PH',
  ADHDPI = 'ADHD-PI',
  None = 'None'
}

export const AdhdTypeMapping: Record<AdhdType, string> = {
  [AdhdType.All]: "All",
  [AdhdType.ADHDPH]: "Predominantly Hyperactive",
  [AdhdType.ADHDPI]: "Predominantly Inattentive",
  [AdhdType.None]: "None"
}

export enum AgeRange {
  All = 'All',
  SeventeenToTwentyNine = '17-29',
  ThirtyToThirtyNine = '30-39',
  FortyToFortyNine = '40-49',
  FiftyToSixtySeven = '50-67',
}

export const AgeRangeMapping: Record<AgeRange, string> = {
  [AgeRange.All]: "All",
  [AgeRange.SeventeenToTwentyNine]: '17-29',
  [AgeRange.ThirtyToThirtyNine]: '30-39',
  [AgeRange.FortyToFortyNine]: '40-49',
  [AgeRange.FiftyToSixtySeven]: '50-67',
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

export class PredictRequest {
  constructor(
    public condition: string,
    public gender: string,
    public age_ranges: string[],
    public adhd_type: string,
    public with_controls: boolean
  ){}
}
