import { CommonModule } from '@angular/common';
import { ChangeDetectorRef, Component, CUSTOM_ELEMENTS_SCHEMA, inject } from '@angular/core';
import { AgGridModule } from 'ag-grid-angular';
import { PlotlyModule } from 'angular-plotly.js';
import { ExploreDataService } from '@services/explore-data.service';
import { BehaviorSubject, Subscription } from 'rxjs';
import { PlotlyResponse, TableData } from '@models/responses';
import { ExploreNavBarComponent } from './explore-nav-bar/explore-nav-bar.component';
import { DemographicCategory, DemographicsRequest, MentalHealthCategory, MentalHealthRequest } from '@models/requests';
import { GridComponent } from '@shared/grid/grid.component';

@Component({
  selector: 'app-explore',
  standalone: true,
  imports: [CommonModule, AgGridModule, PlotlyModule, ExploreNavBarComponent, GridComponent],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  providers: [ExploreDataService],
  templateUrl: './explore.component.html',
  styleUrl: './explore.component.scss'
})
export class ExploreComponent {

  private dataService = inject(ExploreDataService);

  public selectedCategory$: BehaviorSubject<string> = new BehaviorSubject<string>('Demographics');
  public selectedVisualization$: BehaviorSubject<string> = new BehaviorSubject<string>('Graph');
  public categorySubscription$: Subscription | undefined;

  public plotlyData$: BehaviorSubject<PlotlyResponse | null> = new BehaviorSubject<PlotlyResponse | null>(null);
  public agGridData$: BehaviorSubject<TableData | null> = new BehaviorSubject<TableData | null>(null);

  public tableCreated = true;
  public plotCreated = true;

  constructor(private changeDetector: ChangeDetectorRef) {}

  ngOnInit() {
    this.updateDemographicsPlotData(DemographicCategory.ADHDSubtypesByAgeGroup, false);
    this.updateDemographicsTableData(DemographicCategory.ADHDSubtypesByAgeGroup, false);

    this.categorySubscription$ = this.selectedCategory$.subscribe((category) => {
      if (category === 'Demographics') {
        this.updateDemographicsPlotData(DemographicCategory.ADHDSubtypesByAgeGroup, false);
        this.updateDemographicsTableData(DemographicCategory.ADHDSubtypesByAgeGroup, false);
      } else {
        this.updateMentalHealthPlotData(MentalHealthCategory.All, false);
        this.updateMentalHealthTableData(MentalHealthCategory.All, false);
      }
    });
  }

  ngOnDestroy() {
    this.categorySubscription$?.unsubscribe();
    this.selectedCategory$.complete();
    this.selectedVisualization$.complete();
  }

  // Drives the visualization change in the template.
  onVisualizationChanged($event: string) {
    this.selectedVisualization$.next($event);
  }

  onCategoryChanged($event: string) {
    this.selectedCategory$.next($event);
  }

  submit($event: DemographicsRequest | MentalHealthRequest) {
    const request = $event instanceof DemographicsRequest ? $event as DemographicsRequest : $event as MentalHealthRequest;
      // TODO: This is still pretty hacky. Both controls should be synced with the category - but not necessarily all the subqueries.
      switch (request.display) {
        case 'Graph':
          if (request instanceof DemographicsRequest) {
            this.updateDemographicsPlotData(request.sub_category, request.with_controls);
          } else {
            this.updateMentalHealthPlotData(request.sub_category, request.with_controls);
          }
          break;
        case 'Table':
          if (request instanceof DemographicsRequest) {
            this.updateDemographicsTableData(request.sub_category, request.with_controls);
          } else {
            this.updateMentalHealthTableData(request.sub_category, request.with_controls);
          }
      }
  }

  private updateDemographicsTableData(subCategory: string, includeControls: boolean) {
    this.detachComponent('Table');
      this.dataService.getDemographicsTableData$(subCategory.toLocaleLowerCase(), '', includeControls).subscribe((res) => {
          this.agGridData$.next(res);
      });
    this.toggleTable();
  }

  private updateDemographicsPlotData(subCategory: string, includeControls: boolean) {
    this.detachComponent('Graph');
    this.dataService.plotDemographics$(subCategory, '', includeControls).subscribe((res) => {
      this.plotlyData$.next(res);
    });
    this.togglePlot();
  }

  private updateMentalHealthTableData(subCategory: string, includeControls: boolean) {
    this.detachComponent('Table');
    this.dataService.getMentalHealthTableData$(subCategory, '', includeControls).subscribe((res) => {
      this.agGridData$.next(res);
    });
    this.toggleTable();
    this.changeDetector.detectChanges();
  }

  private updateMentalHealthPlotData(subCategory: string, includeControls: boolean) {
    this.detachComponent('Graph');
    this.dataService.plotMentalHealth$(subCategory, '', includeControls).subscribe((res) => {
      this.plotlyData$.next(res);
    });
    this.togglePlot();
    this.changeDetector.detectChanges();
  }

  private detachComponent(graphOrTable: string) {
    if (graphOrTable === 'Graph') {
      this.togglePlot();
    } else {
      this.toggleTable();
    }

    this.changeDetector.detectChanges();
  }

  private togglePlot() {
    this.plotCreated = !this.plotCreated;
  }

  private toggleTable() {
    this.tableCreated = !this.tableCreated;
  }
}
