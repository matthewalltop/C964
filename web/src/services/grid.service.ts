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

  public createGridOptionsAndRowData(data: Observable<TableData>): Observable<[GridOptions, any[]]> {
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
          paginationAutoPageSize: true
        };

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

        return [options, rowData];
      }));
  }
}
