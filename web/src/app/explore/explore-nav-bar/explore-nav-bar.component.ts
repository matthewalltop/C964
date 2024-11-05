import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Output } from '@angular/core';
import { ExploreDataCategory, ExploreDataCategoryMapping, VisualizationOptions, VisualizationOptionsMapping } from '../../../models/requests';

@Component({
  selector: 'app-explore-nav-bar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './explore-nav-bar.component.html',
  styleUrl: './explore-nav-bar.component.scss'
})
export class ExploreNavBarComponent {
  @Output() selectedCategory = new EventEmitter<string>();
  @Output() selectedVisualization = new EventEmitter<string>();
  @Output() includeControls = new EventEmitter<boolean>();

  public categoryMapping = ExploreDataCategoryMapping;
  public categories = Object.values(ExploreDataCategory);

  public visualizationMapping = VisualizationOptionsMapping;
  public visualizations = Object.values(VisualizationOptions);

  public includeControlsValue = false;

  changeVisualization(event: any) {
    const visualSelection: string = event.target.value;
    this.selectedVisualization.emit(visualSelection);
  }

  changeCategory(event: any) {
    const categorySelection = event.target.value;
    this.selectedCategory.emit(categorySelection);
  }

  toggleControls() {
    this.includeControlsValue = !this.includeControlsValue;
    this.includeControls.emit(this.includeControlsValue);
  }
}
