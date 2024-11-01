import { inject, Injectable } from '@angular/core';
import { GridService } from './grid.service';
import { PlotService } from './plot.service';
import { GridOptions } from 'ag-grid-community';
import { Observable, of, switchMap } from 'rxjs';
import { HttpParams } from '@angular/common/http';
import { PlotlyGraph } from '../models/responses';

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

  plotADHDSubtypesByGender(gender: string): Observable<PlotlyGraph> {
    // TODO: This is the generic graph format for everything.
    // The deserialization nonsense can be pulled out into a utility method
    return this.plotService.getPlot('demographic', new HttpParams().set("gender", gender));
  };

  plotADHDSubtypesByAge(): Observable<PlotlyGraph> {
    // TODO: This is the generic graph format for everything.
    // The deserialization nonsense can be pulled out into a utility method
    return this.plotService.getPlot('demographic');
  };


}
