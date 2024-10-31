import { TestBed } from '@angular/core/testing';

import { SubtypeService } from './subtype.service';

describe('SubtypeService', () => {
  let service: SubtypeService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SubtypeService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
