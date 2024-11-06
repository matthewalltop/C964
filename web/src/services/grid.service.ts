import { inject, Injectable } from '@angular/core';
import { environment } from '../environments/environment.development';
import { map, Observable, tap } from 'rxjs';
import { TableData } from '../models/responses';
import { HttpClient, HttpParams } from '@angular/common/http';
import { GridOptions } from 'ag-grid-community';

const baseUrl = environment.api.base;

@Injectable({
  providedIn: 'root'
})
export class GridService {
  private http = inject(HttpClient);

  constructor() { }

  getTable(endpoint: string, params?: HttpParams | undefined): Observable<TableData> {
    return this.http.get(`${baseUrl}/${endpoint}`, { params }).pipe(
      map((res) => JSON.stringify(res)),
      map((res) => JSON.parse(res)),
      map((table) => {
        return {
          columns: table.columns.map((x: any) => x)
        };
      })
    );
  }

  public formatGridOptions(data: Observable<TableData>): Observable<GridOptions> {
    return data.pipe(
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
          // TODO: These need to go into a component.
          // onGridReady: (params: GridReadyEvent) => {
          //   this.gridApi = params.api;
          //   this.gridApi?.sizeColumnsToFit();
          // },
          // onGridSizeChanged: () => {
          //   this.gridApi?.sizeColumnsToFit();
        };

        return options;
      }));
  }
}
