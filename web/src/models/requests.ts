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
  None = 'None',
  ADHDPH = 'ADHD-PH',
  ADHDPI = 'ADHD-PI',
  All = 'All'
}

export const AdhdTypeMapping: Record<AdhdType, string> = {
  [AdhdType.None]: "None",
  [AdhdType.ADHDPH]: "Predominantly Hyperactive",
  [AdhdType.ADHDPI]: "Predominantly Inattentive",
  [AdhdType.All]: "All"
}

export enum AgeRange {
  SeventeenToTwentyNine = '17-29',
  ThirtyToThirtyNine = '30-39',
  FortyToFortyNine = '40-49',
  FiftyToSixtySeven = '50-67',
  All = 'All',
}

export const AgeRangeMapping: Record<AgeRange, string> = {
  [AgeRange.SeventeenToTwentyNine]: '17-29',
  [AgeRange.ThirtyToThirtyNine]: '30-39',
  [AgeRange.FortyToFortyNine]: '40-49',
  [AgeRange.FiftyToSixtySeven]: '50-67',
  [AgeRange.All]: "All",
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
    public include_controls: boolean
  ){}
}
