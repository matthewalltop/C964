import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Output } from '@angular/core';
import { BehaviorSubject, combineLatest } from 'rxjs';
import {
  AdhdType,
  AdhdTypeMapping,
  AgeRange,
  AgeRangeMapping,
  Gender,
  GenderMapping,
  MentalHealthCategory,
  MentalHealthCategoryMapping,
  PredictRequest
} from '@models/requests';

@Component({
  selector: 'app-predict-nav-bar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './predict-nav-bar.component.html',
  styleUrl: './predict-nav-bar.component.scss'
})
export class PredictNavBarComponent {
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
  }

  changeGender(event: any) {
    const genderSelection = event.target.value;
    this.genderCategory$.next(genderSelection);
  }

  changeAgeRanges(event: any) {
    const ageRangeCategories = Array.from(event.target.selectedOptions).map((option: any) => option.value);
    if (ageRangeCategories.includes("All")) {
      this.ageRangeCategories$.next(["All"]);
      return;
    }
    this.ageRangeCategories$.next(ageRangeCategories);
  }

  changeAdhdType(event: any) {
    const adhdTypeSelection = event.target.value;
    this.adhdTypeCategory$.next(adhdTypeSelection);
  }

  toggleControls() {
    this.includeControlsValue = !this.includeControlsValue;
  }

  submit(event: any) {
    combineLatest([this.comorbidity$, this.genderCategory$, this.ageRangeCategories$, this.adhdTypeCategory$])
      .subscribe(([comorbidity, gender, ageRanges, adhdType]) => {
      this.search.emit(new PredictRequest(comorbidity, gender, ageRanges, adhdType, this.includeControlsValue));
    });
  }
}
