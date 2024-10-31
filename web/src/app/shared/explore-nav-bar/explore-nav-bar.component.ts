import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Output } from '@angular/core';

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

  // TODO: These may be better suited as static exports from external files
  public visualizationCategories = new Set<string>([
    'ADHD Subtypes',
    'Demographics',
    'Mental Health',
    'Medication'
  ]);

  public visualizationOptions = new Set<string>([
    'Visual',
    'Raw Data'
  ]);

  changeVisualization(event: any) {
    const visualSelection: string = event.target.value;
    if (this.visualizationOptions.has(visualSelection)) {
      this.selectedVisualization.emit(visualSelection);
    }
  }

  changeCategory(event: any) {
    const categorySelection: string = event.target.value;
    if (this.visualizationCategories.has(categorySelection)) {
      this.selectedCategory.emit(categorySelection);
    }
  }
}
