import { TestBed } from '@angular/core/testing';
import { ExploreDataService } from './explore-data.service';

describe('ExploreDataService', () => {
  let service: ExploreDataService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ExploreDataService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
