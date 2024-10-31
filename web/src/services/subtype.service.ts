import { inject, Injectable } from '@angular/core';
import { GridService } from './grid.service';
import { PlotService } from './plot.service';
import { GridOptions } from 'ag-grid-community';
import { Observable, of, switchMap } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class SubtypeService {
  private gridService = inject(GridService);
  private plotService = inject(PlotService);

  constructor() { }

  getADHDSubtypesWithGenderAndAgeData(): Observable<GridOptions> {
    return this.gridService.getTable('subtype').pipe(
      switchMap((res) => this.gridService.formatGridOptions(of(res))));
  };
}
