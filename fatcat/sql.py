
import json
import random
import hashlib
from fatcat import db
from fatcat.models import *

def populate_db():
    """
    TODO: doesn't create an edit trail (yet)
    """

    n_elkies = CreatorRevision(
        name="Noam D. Elkies",
        sortname="Elkies, N",
        orcid=None)
    n_elkies_id = CreatorId(revision_id=n_elkies.id)
    pi_work = WorkRevision(
        title="Why is π 2so close to 10?")
    pi_work_id = WorkId(revision_id=pi_work.id)
    pi_release = ReleaseRevision(
        title=pi_work.title,
        work_id=pi_work.id)
    pi_contrib = ReleaseContrib(creator=n_elkies_id)
    pi_release.creators.append(pi_contrib)
    pi_release_id = ReleaseId(revision_id=pi_release.id)
    pi_work.primary_release = pi_release.id

    # TODO:
    #pi_file = File(
    #    sha1="efee52e46c86691e2b892dbeb212f3b92e92e9d3",
    #    url="http://www.math.harvard.edu/~elkies/Misc/pi10.pdf")
    db.session.add_all([n_elkies, pi_work, pi_work_id, pi_release, pi_release_id])

    # TODO:
    #ligo_collab = CreatorRevision(name="LIGO Scientific Collaboration")
    #ligo_paper = ReleaseRevision(
    #    title="Full Band All-sky Search for Periodic Gravitational Waves in the O1 LIGO Data")
    db.session.commit()


def populate_complex_db(count=100):
    """
    TODO: doesn't create an edit trail (yet)
    """

    first_names = ("Sarah", "Robin", "Halko", "Jefferson", "Max", "桃井",
        "Koizumi", "Rex", "Billie", "Tenzin")
    last_names = ("Headroom", "はるこ", "Jun'ichirō", "Wong", "Smith")

    author_revs = []
    author_ids = []
    for _ in range(count):
        first = random.choice(first_names)
        last = random.choice(last_names)
        ar = CreatorRevision(
            name="{} {}".format(first, last),
            sortname="{}, {}".format(last, first[0]),
            orcid=None)
        author_revs.append(ar)
        author_ids.append(CreatorId(revision_id=ar.id))

    container_revs = []
    container_ids = []
    for _ in range(5):
        cr = ContainerRevision(
            name="The Fake Journal of Stuff",
            container_id=None,
            publisher="Big Paper",
            sortname="Fake Journal of Stuff",
            issn="1234-5678")
        container_revs.append(cr)
        container_ids.append(ContainerId(revision_id=cr.id))

    title_start = ("All about ", "When I grow up I want to be",
        "The final word on", "Infinity: ", "The end of")
    title_ends = ("Humankind", "Bees", "Democracy", "Avocados", "«küßî»", "“ЌύБЇ”")
    work_revs = []
    work_ids = []
    release_revs = []
    release_ids = []
    file_revs = []
    file_ids = []
    for _ in range(count):
        title = "{} {}".format(random.choice(title_start), random.choice(title_ends))
        work = WorkRevision(title=title)
        work_id = WorkId(revision_id=work.id)
        authors = set(random.sample(author_ids, 5))
        release = ReleaseRevision(
            title=work.title,
            creators=[ReleaseContrib(creator=a) for a in list(authors)],
            work_id=work.id,
            container_id=random.choice(container_ids).id)
        release_id = ReleaseId(revision_id=release.id)
        work.primary_release = release.id
        authors.add(random.choice(author_ids))
        release2 = ReleaseRevision(
            title=work.title + " (again)",
            creators=[ReleaseContrib(creator=a) for a in list(authors)],
            work_id=work.id,
            container_id=random.choice(container_ids).id)
        release_id2 = ReleaseId(revision_id=release2.id)
        work_revs.append(work)
        work_ids.append(work_id)
        release_revs.append(release)
        release_revs.append(release2)
        release_ids.append(release_id)
        release_ids.append(release_id2)

        file_content = str(random.random()) * random.randint(3,100)
        file_sha = hashlib.sha1(file_content.encode('utf-8')).hexdigest()
        file_rev = FileRevision(
            sha1=file_sha,
            size=len(file_content),
            url="http://archive.invalid/{}".format(file_sha),
            releases=[FileRelease(release=release_id), FileRelease(release=release_id2)],
        )
        file_id = FileId(revision_id=file_rev.id)
        file_revs.append(file_rev)
        file_ids.append(file_id)

    db.session.add_all(author_revs)
    db.session.add_all(author_ids)
    db.session.add_all(work_revs)
    db.session.add_all(work_ids)
    db.session.add_all(release_revs)
    db.session.add_all(release_ids)
    db.session.add_all(container_revs)
    db.session.add_all(container_ids)
    db.session.add_all(file_revs)
    db.session.add_all(file_ids)

    db.session.commit()

def add_crossref(meta):

    title = meta['title'][0]

    # authors
    author_revs = []
    author_ids = []
    for am in meta['author']:
        ar = CreatorRevision(
            name="{} {}".format(am['given'], am['family']),
            sortname="{}, {}".format(am['family'], am['given']),
            orcid=None)
        author_revs.append(ar)
        author_ids.append(CreatorId(revision_id=ar.id))

    # container
    container = ContainerRevision(
        issn=meta['ISSN'][0],
        name=meta['container-title'][0],
        container_id=None,
        publisher=meta['publisher'],
        sortname=meta['short-container-title'][0])
    container_id = ContainerId(revision_id=container.id)

    # work and release
    work = WorkRevision(title=title)
    work_id = WorkId(revision_id=work.id)
    release = ReleaseRevision(
        title=title,
        creators=[ReleaseContrib(creator=a) for a in author_ids],
        work_id=work.id,
        container_id=container_id.id,
        release_type=meta['type'],
        doi=meta['DOI'],
        date=meta['created']['date-time'],
        license=meta.get('license', [dict(URL=None)])[0]['URL'] or None,
        issue=meta.get('issue', None),
        volume=meta.get('volume', None),
        pages=meta.get('page', None))
    release_id = ReleaseId(revision_id=release.id)
    work.primary_release = release.id
    extra = json.dumps({
        'crossref': {
            'links': meta.get('link', []),
            'subject': meta['subject'],
            'type': meta['type'],
            'alternative-id': meta.get('alternative-id', []),
        }
    }, indent=None).encode('utf-8')
    extra_json = ExtraJson(json=extra, sha1=hashlib.sha1(extra).hexdigest())
    release.extra_json = extra_json.sha1

    # references
    for i, rm in enumerate(meta.get('reference', [])):
        ref = ReleaseRef(
            release_rev=release.id,
            doi=rm.get("DOI", None),
            index=i+1,
            # TODO: how to generate a proper stub here from k/v metadata?
            stub="| ".join(rm.values()))
        release.refs.append(ref)

    db.session.add_all([work, work_id, release, release_id, container,
        container_id, extra_json])
    db.session.add_all(author_revs)
    db.session.add_all(author_ids)
    db.session.commit()
