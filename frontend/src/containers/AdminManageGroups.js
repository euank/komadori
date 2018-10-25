import { connect } from 'react-redux';
import { doAdminListGroups, doAdminCreateGroup } from '../actions';
import AdminManageGroupsComponent from '../components/AdminManageGroups';

const mapStateToProps = state => ({
  groups: state.admin.groups,
});

const mapDispatchToProps = dispatch => ({
  adminListGroups: () => {
    dispatch(doAdminListGroups());
  },
  adminCreateGroup: (group) => {
    dispatch(doAdminCreateGroup(group));
  },
});

const AdminManageGroups = connect(
  mapStateToProps,
  mapDispatchToProps,
)(AdminManageGroupsComponent);

export default AdminManageGroups;
