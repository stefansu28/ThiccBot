"""empty message

Revision ID: 9e2734465d2b
Revises: 48d14a2a4cd8
Create Date: 2018-12-28 04:50:35.866892

"""
from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision = '9e2734465d2b'
down_revision = '48d14a2a4cd8'
branch_labels = None
depends_on = None


def upgrade():
    # ### commands auto generated by Alembic - please adjust! ###
    op.alter_column('alias', 'server_id',
               existing_type=sa.INTEGER(),
               type_=sa.BIGINT(),
               existing_nullable=True)
    # ### end Alembic commands ###


def downgrade():
    # ### commands auto generated by Alembic - please adjust! ###
    op.alter_column('alias', 'server_id',
               existing_type=sa.BIGINT(),
               type_=sa.INTEGER(),
               existing_nullable=True)
    # ### end Alembic commands ###
