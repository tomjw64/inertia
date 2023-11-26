const addCross = (result: string[], namePairing: NamePairing) => {
  const { names, descriptors } = namePairing;
  for (const name of names) {
    for (const descriptor of descriptors) {
      result.push(descriptor + ' ' + name);
    }
  }
};

const collectNames = (namePairings: NamePairing[]) => {
  const result = [];
  for (const pairing of namePairings) {
    addCross(result, pairing);
  }
  return result;
};

type NamePairing = { names: string[]; descriptors: string[] };
const NAME_PAIRINGS: NamePairing[] = [
  { names: [], descriptors: [] },
  {
    names: ['NOG', 'NAGATA', 'NAUSICAAN'],
    descriptors: ['NICE', 'NEARLY', 'NOBLE', 'NATURALLY'],
  },
  {
    names: ['GORN', 'GUNGAN', 'GERRERA', 'GAMORREAN'],
    descriptors: ['GOOGLY', 'GREAT', 'GRITTY', 'GLOBAL', 'GIGGLY'],
  },
  {
    names: ['ERSO', 'EWOK', 'EZRA'],
    descriptors: ['ECCENTRIC', 'EQUALLY', 'ELATED', 'EAGER', 'EXACTLY'],
  },
  {
    names: [
      'GEORDI',
      'JANEWAY',
      'JAYNE',
      'JALAD',
      'JAVA',
      'GEONOSIAN',
      'DJARIN',
      'JAR JAR',
    ],
    descriptors: [
      'JARRING',
      'JUST',
      'JOLLY',
      'JOVIAL',
      'JAZZY',
      'GENIAL',
      'GENUINELY',
    ],
  },
  {
    names: ['HOLDEN', 'HAN', 'HONDO', 'HUTT'],
    descriptors: ['HARDLY', 'HELPFUL', 'HEALTHY', 'HUMBLE', 'HANDY'],
  },
  {
    names: [
      'BO-KATAN',
      'BORG',
      'BRAND',
      'BOSSK',
      'BALTAR',
      'BASHIR',
      'BAJORAN',
      'BETAZOID',
    ],
    descriptors: ['BARELY', 'BOLDLY', 'BRAVE', 'BIZARRE', 'BASHFUL', 'BITTER'],
  },
  {
    names: ['TROI', 'TARS', 'TARKIN', 'TUVOK', 'TIGH', 'TRILL', 'TRANDOSHAN'],
    descriptors: ['TOTALLY', 'TIRED', 'TRANQUIL', 'TACKY', 'TACTICAL', 'TRULY'],
  },
  {
    names: ['DOOKU', 'DATA', 'DAX', 'DRAPER', 'DRUMMER', 'DARMOK'],
    descriptors: [
      'DOWNRIGHT',
      'DEFINITELY',
      'DORKY',
      'DARING',
      'DOZY',
      'DECENTLY',
      'DISTINCTLY',
    ],
  },
  {
    names: ['ADAMA', 'ARCHER', 'AHSOKA', 'ANAKIN', 'ACKBAR', 'ANDOR'],
    descriptors: ['ANGRY', 'AVID', 'AGILE', 'AWKWARD', 'ALSO', 'ACTUALLY'],
  },
  {
    names: [
      'SPOCK',
      'SULU',
      'SOLO',
      'SISKO',
      'STARBUCK',
      'SKYWALKER',
      'SCOTTY',
    ],
    descriptors: [
      'SUPERB',
      'SPICY',
      'SNUGGLY',
      'SAVVY',
      'SWEET',
      'SORTA',
      'SILLY',
      'SUDDENLY',
    ],
  },
  {
    names: ['KYLO', 'KENOBI', 'KIRK', 'KAYLEE', 'KLINGON', 'KANAN', 'KAMINOAN'],
    descriptors: ['KEEN', 'KIND', 'KINETIC', 'KINGLY', 'KILLER', 'KINDA'],
  },
  {
    names: ['CRUSHER', 'COOPER', 'CASE', 'CARDASSIAN'],
    descriptors: ['CALM', 'COOL', 'CARING', 'CAPABLE', 'CLEARLY'],
  },
  {
    names: ['VADER', 'VULCAN', 'VENTRESS'],
    descriptors: ['VIBRANT', 'VERMILLION', 'VEXED', 'VERY', 'VERITABLY'],
  },
  {
    names: ['RIKER', 'RO', 'REY', 'ROMULAN', 'RODIAN'],
    descriptors: [
      'ROTUND',
      'ROBUST',
      'RUDE',
      'RELIABLE',
      'RAGGED',
      'ROWDY',
      'REALLY',
      'RATHER',
      'REMOTELY',
      'RARE',
    ],
  },
  {
    names: ['QUI-GON', 'Q', 'QUARK'],
    descriptors: ['QUIET', 'QUICK', 'QUALITY', 'QUAINT', 'QUIRKY', 'QUITE'],
  },
  {
    names: ['PICARD', 'PIKE', 'PADME', 'PO', 'PALPATINE', 'PLO KOON'],
    descriptors: [
      'PROPER',
      'PROUD',
      'PESKY',
      'PRACTICALLY',
      'PRECISELY',
      'PRETTY',
      'PERFECTLY',
    ],
  },
  {
    names: ['MAL', 'MOTHMA', 'MAUL', 'MCCOY', 'MURPH', 'MILLER'],
    descriptors: [
      'MOSTLY',
      'MAINLY',
      'MACHO',
      'MAGIC',
      'MARVELOUS',
      'MERRY',
      'MEAN',
      'MIGHTY',
    ],
  },
  {
    names: ['WINDU', 'WORF', 'WOOKIEE', 'WASH'],
    descriptors: [
      'WONDROUS',
      'WACKY',
      'WORTHY',
      'WILD',
      'WIMPY',
      'WOBBLY',
      'WISE',
      'WITTY',
      'WIGGLY',
      'WISTFUL',
      'WEIRD',
    ],
  },
];

const COLLECTED_NAMES = collectNames(NAME_PAIRINGS);
console.log('Possible names:', COLLECTED_NAMES.length);

export const generatePlayerName = () => {
  const index = Math.floor(Math.random() * COLLECTED_NAMES.length);
  return COLLECTED_NAMES[index];
};

export const generatePlayerId = () => {
  return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
};

export const generatePlayerReconnectKey = () => {
  return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
};
