import { CommonModule } from '@angular/common';
import { Component, inject, Input } from '@angular/core';
import { AgGridModule } from 'ag-grid-angular';
import { GridApi, GridOptions, GridReadyEvent } from 'ag-grid-community';
import { BehaviorSubject, of } from 'rxjs';
import { GridService } from '../../../services/grid.service';
import { TableData } from '../../../models/responses';

@Component({
  selector: 'app-grid',
  standalone: true,
  imports: [CommonModule, AgGridModule],
  templateUrl: './grid.component.html',
  styleUrl: './grid.component.scss'
})
export class GridComponent {
  @Input() public data: TableData | undefined;

  public tableService = inject(GridService);
  public gridApi: GridApi | undefined;

  public gridOptions$: BehaviorSubject<GridOptions> = new BehaviorSubject<GridOptions>({});
  public rowData$: BehaviorSubject<any[] | undefined> = new BehaviorSubject<any[] | undefined>(undefined);

  ngOnInit() {
    this.tableService.createGridOptionsAndRowData(of(this.data!)).subscribe(([options, rowData]) => {
      options.onGridReady = this.onGridReady.bind(this);
      options.onGridSizeChanged = this.onGridSizeChanged.bind(this);
      this.gridOptions$.next(options);
      this.rowData$.next(rowData);
    });
  }


  onGridReady(params: GridReadyEvent) {
    this.gridApi = params.api;
    this.gridApi?.sizeColumnsToFit();
  }

  onGridSizeChanged() {
    this.gridApi?.sizeColumnsToFit();
  }
}
