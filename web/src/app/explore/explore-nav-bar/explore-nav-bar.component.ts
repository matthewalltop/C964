import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Output } from '@angular/core';
import { DemographicCategory, DemographicCategoryMapping, ExploreDataCategory, ExploreDataCategoryMapping, MentalHealthCategory, MentalHealthCategoryMapping, VisualizationOptions, VisualizationOptionsMapping } from '../../../models/requests';

@Component({
  selector: 'app-explore-nav-bar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './explore-nav-bar.component.html',
  styleUrl: './explore-nav-bar.component.scss'
})
export class ExploreNavBarComponent {
  @Output() selectedCategory = new EventEmitter<string>();
  @Output() selectedSubCategory = new EventEmitter<string>();
  @Output() selectedVisualization = new EventEmitter<string>();
  @Output() includeControls = new EventEmitter<boolean>();

  public categoryMapping = ExploreDataCategoryMapping;
  public categories = Object.values(ExploreDataCategory);

  public demographicCategoryMapping = DemographicCategoryMapping;
  public mentalHealthCategoryMapping = MentalHealthCategoryMapping;
  public demographicSubCateories = Object.values(DemographicCategory);
  public mentalHealthSubCategories = Object.values(MentalHealthCategory);

  public visualizationMapping = VisualizationOptionsMapping;
  public visualizations = Object.values(VisualizationOptions);

  public category: ExploreDataCategory = ExploreDataCategory.Demographics;
  public subCategory: DemographicCategory | MentalHealthCategory = DemographicCategory.ADHDSubtypesByAgeGroup;
  public includeControlsValue = false;

  changeVisualization(event: any) {
    const visualSelection: string = event.target.value;
    this.selectedVisualization.emit(visualSelection);
  }

  changeCategory(event: any) {
    const categorySelection = event.target.value;
    this.category = this.categories[categorySelection];
    this.selectedCategory.emit(categorySelection);
  }

  toggleControls() {
    this.includeControlsValue = !this.includeControlsValue;
    this.includeControls.emit(this.includeControlsValue);
  }

  changeSubCategory(event: any) {
    const subCategorySelection = event.target.value;
    this.subCategory = this.category === ExploreDataCategory.Demographics ?
      this.demographicSubCateories[subCategorySelection] :
      this.mentalHealthSubCategories[subCategorySelection];
    this.selectedSubCategory.emit(subCategorySelection);
  }
}
