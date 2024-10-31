import { CommonModule } from '@angular/common';
import { Component, CUSTOM_ELEMENTS_SCHEMA, inject } from '@angular/core';
import { AgGridModule } from 'ag-grid-angular';
import { PlotlyModule } from 'angular-plotly.js';
import { DemographicService } from '../../services/demographic.service';
import { GridService } from '../../services/grid.service';
import { GridApi, GridOptions, GridReadyEvent } from 'ag-grid-community';
import { Observable, BehaviorSubject, map } from 'rxjs';
import { PlotlyGraph, TableData } from '../../models/responses';
import { ExploreNavBarComponent } from '../shared/explore-nav-bar/explore-nav-bar.component';

@Component({
  selector: 'app-explore',
  standalone: true,
  imports: [CommonModule, AgGridModule, PlotlyModule, ExploreNavBarComponent],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  providers: [DemographicService, GridService],
  templateUrl: './explore.component.html',
  styleUrl: './explore.component.scss'
})
export class ExploreComponent {
  private demographicService = inject(DemographicService);
  private tableService = inject(GridService);

  // TODO: Pull these out into services & abstract the template into re-usable components.
  public plotlyResponse$: Observable<PlotlyGraph> | undefined;
  public tableResponse$: Observable<TableData> | undefined;

  public rowData$: BehaviorSubject<any[] | null> = new BehaviorSubject<any[] | null>(null);

  public gridApi: GridApi | undefined;
  public gridOptions$: Observable<GridOptions> | undefined;

  public selectedVisualization$: BehaviorSubject<string> = new BehaviorSubject<string>('Visual');
  public selectedCategory$: BehaviorSubject<string> = new BehaviorSubject<string>('ADHD Subtypes');

  ngOnInit() {
    this.plotlyResponse$ = this.demographicService.getDemographics();
    this.tableResponse$ = this.tableService.getTable("subtype");

    // TODO: This builds a dynamic ag-grid.
    // This can probably be moved over into a service that just provides these options directly.

    this.gridOptions$ = this.tableResponse$!.pipe(
      map((res) => {

        const options: GridOptions = {
          columnDefs: res.columns.map((column) => {
            return {
              headerName: column.name,
              field: column.name.replace(' ', '_').toLocaleLowerCase()
            };
          }),
          defaultColDef: {
            menuTabs: ['filterMenuTab'],
            filter: 'agTextColumnFilter',
            filterParams: {
              type: 'text',
              filterOptions: ['contains', 'startsWith', 'endsWith'],
              defaultOption: 'contains',
              buttons: ['reset']
            },
            sortable: true,
            resizable: true,
          },
          animateRows: true,
          pagination: true,
          paginationAutoPageSize: true,
          onGridReady: (params: GridReadyEvent) => {
            this.gridApi = params.api;
            this.gridApi?.sizeColumnsToFit();
          },
          onGridSizeChanged: () => {
            this.gridApi?.sizeColumnsToFit();
          },
        }

        const rows = res.columns
          .map((column) => [column.name.replace(' ', '_').toLocaleLowerCase(), column.values]);

        let rowData = [];
        // Create an object with the column name as the key and the values at the current index as the value.
        // When all column names are added, push the object to the rowData array.
        for (let i = 0; i < rows[0][1].length; i++) {
          const row: any = {};
          rows.forEach(([key, values]) => {
            row[key as string] = values[i];
          });
          rowData.push(row);
        }

        this.rowData$.next(rowData);

        return options;
      }));
  }

  onCategoryChanged(category: string) {
    this.selectedCategory$.next(category);
  }

  onVisualizationChanged(visualization: string) {
    this.selectedVisualization$.next(visualization);
  }
}
