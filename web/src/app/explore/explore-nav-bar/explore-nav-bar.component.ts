import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Output } from '@angular/core';
import {
  DemographicCategory,
  DemographicCategoryMapping,
  DemographicsRequest,
  ExploreDataCategory,
  ExploreDataCategoryMapping,
  MentalHealthCategory,
  MentalHealthCategoryMapping,
  MentalHealthRequest,
  VisualizationOptions,
  VisualizationOptionsMapping
} from '@models/requests';
import { BehaviorSubject, combineLatest } from 'rxjs';

@Component({
  selector: 'app-explore-nav-bar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './explore-nav-bar.component.html',
  styleUrl: './explore-nav-bar.component.scss'
})
export class ExploreNavBarComponent {
  @Output() selectedVisualization = new EventEmitter<string>();
  @Output() selectedCategory = new EventEmitter<string>();
  @Output() selectedSubCategory = new EventEmitter<string>();
  @Output() search = new EventEmitter<DemographicsRequest | MentalHealthRequest>();

  public categoryMapping = ExploreDataCategoryMapping;
  public demographicCategoryMapping = DemographicCategoryMapping;
  public mentalHealthCategoryMapping = MentalHealthCategoryMapping;
  public visualizationMapping = VisualizationOptionsMapping;

  public categories = Object.values(ExploreDataCategory);
  public demographicSubCateories = Object.values(DemographicCategory);
  public mentalHealthSubCategories = Object.values(MentalHealthCategory);
  public visualizations = Object.values(VisualizationOptions);

  // To drive dynamic controls
  public visualization$ = new BehaviorSubject<string>("Graph");
  public category$ = new BehaviorSubject<string>("Demographics");
  public demographicSubCategory$ = new BehaviorSubject<string>("Age");
  public mentalHealthSubCategory$ = new BehaviorSubject<string>("All");

  public includeControlsValue = false;

  get visualization() {
    return this.visualization$.value;
  }

  ngOnDestroy() {
    this.visualization$.complete();
    this.category$.complete();
    this.demographicSubCategory$.complete();
    this.mentalHealthSubCategory$.complete();
  }

  changeVisualization(event: any) {
    const visualSelection: string = event.target.value;
    this.visualization$.next(visualSelection);
    this.selectedVisualization.emit(visualSelection);
  }

  changeCategory(event: any) {
    const categorySelection = event.target.value;
    this.category$.next(categorySelection);
    this.selectedCategory.emit(categorySelection);
  }

  changeSubCategory(event: any) {
    const subCategorySelection = event.target.value;

    if (this.category$.value === "Demographics") {
      this.demographicSubCategory$.next(subCategorySelection);
    } else {
      this.mentalHealthSubCategory$.next(subCategorySelection);
    }
    this.selectedSubCategory.emit(subCategorySelection);
  }

  toggleControls() {
    this.includeControlsValue = !this.includeControlsValue;
  }

  submit(event: any) {
    combineLatest([this.visualization$, this.category$, this.demographicSubCategory$, this.mentalHealthSubCategory$]).subscribe(([visualization, category, demographicSubCategory, mentalHealthSubCategory]) => {
      if (category === 'Demographics') {
        const request = new DemographicsRequest(visualization, demographicSubCategory, null, this.includeControlsValue);
        this.search.emit(request);
      } else {
        const request = new MentalHealthRequest(visualization, mentalHealthSubCategory, null, this.includeControlsValue);
        this.search.emit(request);
      }
    }).unsubscribe();
  }
}
