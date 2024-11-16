import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Output } from '@angular/core';
import { BehaviorSubject, combineLatest } from 'rxjs';
import {
  AdhdType,
  AdhdTypeMapping,
  AgeRange,
  AgeRangeMapping,
  Algorithm,
  AlgorithmMapping,
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
  public adhdTypeCategoryMapping = AdhdTypeMapping;
  public algorithmCategoryMapping = AlgorithmMapping;

  public comorbidityCategories = Object.values(MentalHealthCategory);
  public genderCategories = Object.values(Gender);
  public adhdTypeCategories = Object.values(AdhdType);
  public algorithmCategories = Object.values(Algorithm);
  public split_ratio = [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1];

  // To drive dynamic controls
  public comorbidity$ = new BehaviorSubject<string>("All");
  public genderCategory$ = new BehaviorSubject<string>("All");
  public adhdTypeCategory$ = new BehaviorSubject<string>("All");
  public algorithm$ = new BehaviorSubject<string>("LogisticRegression");
  public split_ratio$ = new BehaviorSubject<number>(0.7);


  changeComorbidity(event: any) {
    const comorbiditySelection = event.target.value;
    this.comorbidity$.next(comorbiditySelection);
  }

  changeGender(event: any) {
    const genderSelection = event.target.value;
    this.genderCategory$.next(genderSelection);
  }

  changeAdhdType(event: any) {
    const adhdTypeSelection = event.target.value;
    this.adhdTypeCategory$.next(adhdTypeSelection);
  }

  changeAlgorithm(event: any) {
    const algorithmSelection = event.target.value;
    this.algorithm$.next(algorithmSelection);
  }

  changeSplitRatio(event: any) {
    const splitRatio = event.target.value;
    this.split_ratio$.next(splitRatio);
  }

  submit(event: any) {
    combineLatest([this.comorbidity$, this.genderCategory$, this.adhdTypeCategory$, this.algorithm$, this.split_ratio$])
      .subscribe(([comorbidity, gender, adhdType, algorithm, split_ratio]) => {
      this.search.emit(new PredictRequest(comorbidity, gender, adhdType, algorithm, split_ratio));
    });
  }
}
