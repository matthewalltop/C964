import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Output } from '@angular/core';
import { AdhdType, AdhdTypeMapping, AgeRange, AgeRangeMapping, DemographicCategory, DemographicCategoryMapping, DemographicsRequest, ExploreDataCategory, ExploreDataCategoryMapping, Gender, GenderMapping, MentalHealthCategory, MentalHealthCategoryMapping, MentalHealthRequest, PredictRequest, VisualizationOptions, VisualizationOptionsMapping } from '../../../models/requests';
import { BehaviorSubject, combineLatest } from 'rxjs';

@Component({
  selector: 'app-predict-nav-bar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './predict-nav-bar.component.html',
  styleUrl: './predict-nav-bar.component.scss'
})
export class PredictNavBarComponent {
  @Output() selectedComorbidity = new EventEmitter<string>();
  @Output() selectedGender = new EventEmitter<string>();
  @Output() selectedAgeRanges = new EventEmitter<string[]>();
  @Output() selectedAdhdType = new EventEmitter<string>();
  @Output() includeControls = new EventEmitter<boolean>();

  @Output() search = new EventEmitter<PredictRequest>();

  public comorbidityCategoryMapping = MentalHealthCategoryMapping;
  public genderCategoryMapping = GenderMapping;
  public ageRangeCategoryMapping = AgeRangeMapping;
  public adhdTypeCategoryMapping = AdhdTypeMapping;

  public comorbidityCategories = Object.values(MentalHealthCategory);
  public genderCategories = Object.values(Gender);
  public ageRangeCategories = Object.values(AgeRange);
  public adhdTypeCategories = Object.values(AdhdType);

  // To drive dynamic controls
  public comorbidity$ = new BehaviorSubject<string>("All");
  public genderCategory$ = new BehaviorSubject<string>("All");
  public ageRangeCategories$ = new BehaviorSubject<string[]>(["All"]);
  public adhdTypeCategory$ = new BehaviorSubject<string>("All");

  public includeControlsValue = false;


  changeComorbidity(event: any) {
    const comorbiditySelection = event.target.value;
    this.comorbidity$.next(comorbiditySelection);
    this.selectedComorbidity.emit(comorbiditySelection);
  }

  changeGender(event: any) {
    const genderSelection = event.target.value;
    this.genderCategory$.next(genderSelection);
    this.selectedGender.emit(genderSelection);
  }

  changeAgeRanges(event: any) {
    const ageRangeCategories = Array.from(event.target.selectedOptions).map((option: any) => option.value);
    if (ageRangeCategories.includes("All")) {
      this.ageRangeCategories$.next(["All"]);
      this.selectedAgeRanges.emit(["All"]);
      return;
    }
    this.ageRangeCategories$.next(ageRangeCategories);
    this.selectedAgeRanges.emit(ageRangeCategories);
  }

  changeAdhdType(event: any) {
    const adhdTypeSelection = event.target.value;
    this.adhdTypeCategory$.next(adhdTypeSelection);
    this.selectedAdhdType.emit(adhdTypeSelection);
  }

  toggleControls() {
    this.includeControlsValue = !this.includeControlsValue;
    this.includeControls.emit(this.includeControlsValue);
  }

  submit(event: any) {
    combineLatest([this.comorbidity$, this.genderCategory$, this.ageRangeCategories$, this.adhdTypeCategory$])
      .subscribe(([comorbidity, gender, ageRanges, adhdType]) => {
      this.search.emit(new PredictRequest(comorbidity, gender, ageRanges, adhdType, this.includeControlsValue));
    });
  }
}
