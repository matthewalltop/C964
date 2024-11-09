import { CommonModule } from '@angular/common';
import { Component, CUSTOM_ELEMENTS_SCHEMA, inject } from '@angular/core';
import { AgGridModule } from 'ag-grid-angular';
import { PlotlyModule } from 'angular-plotly.js';
import { ExploreDataService } from '../../services/explore-data.service';
import { GridService } from '../../services/grid.service';
import { BehaviorSubject } from 'rxjs';
import { PlotlyGraph, TableData } from '../../models/responses';
import { ExploreNavBarComponent } from './explore-nav-bar/explore-nav-bar.component';
import { DemographicCategory, DemographicsRequest, MentalHealthRequest } from '../../models/requests';
import { GridComponent } from '../shared/grid/grid.component';

@Component({
  selector: 'app-explore',
  standalone: true,
  imports: [CommonModule, AgGridModule, PlotlyModule, ExploreNavBarComponent, GridComponent],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  providers: [ExploreDataService, GridService],
  templateUrl: './explore.component.html',
  styleUrl: './explore.component.scss'
})
export class ExploreComponent {
  private dataService = inject(ExploreDataService);

  public selectedVisualization$: BehaviorSubject<string> = new BehaviorSubject<string>('Graph');
  public plotlyData$: BehaviorSubject<PlotlyGraph | null> = new BehaviorSubject<PlotlyGraph | null>(null);
  public agGridData$: BehaviorSubject<TableData | null> = new BehaviorSubject<TableData | null>(null);

  ngOnInit() {
    this.updateDemographicsPlotData(DemographicCategory.ADHDSubtypesByAgeGroup, false);
    this.updateDemographicsTableData(DemographicCategory.ADHDSubtypesByAgeGroup, false);
  }

  // Drives the visualization change in the template.
  onVisualizationChanged($event: string) {
    this.selectedVisualization$.next($event);
  }

  submit($event: DemographicsRequest | MentalHealthRequest) {
    const request = typeof $event === typeof DemographicsRequest ? $event as DemographicsRequest : $event as MentalHealthRequest;

    // TODO: Can be cleaned up more.
    if (typeof request === typeof DemographicsRequest) {
      if (request.display === 'Graph') {
          this.updateDemographicsPlotData(request.sub_category, request.with_controls);
      } else {
          this.updateDemographicsTableData(request.sub_category, request.with_controls);
      }
    } else {
        if (request.display === 'Graph') {
          this.updateDemographicsPlotData(request.sub_category, request.with_controls);
      } else {
          this.updateDemographicsTableData(request.sub_category, request.with_controls);
      }
    }
  }

  private updateDemographicsTableData(subCategory: string, includeControls: boolean) {
      this.dataService.getDemographicsTableData$(subCategory.toLocaleLowerCase(), '', includeControls).subscribe((res) => {
          this.agGridData$.next(res);
        });
  }

  private updateDemographicsPlotData(subCategory: string, includeControls: boolean) {
    this.dataService.plotDemographics$(subCategory, '', includeControls).subscribe((res) => {
      this.plotlyData$.next(res);
    });
  }
}
